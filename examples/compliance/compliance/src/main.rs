use futures::{Future, TryStreamExt};
use hdp_sdk::DataProcessorClient;
use reth_exex::{ExExContext, ExExEvent, ExExNotification};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_tracing::tracing::info;

/// The initialization logic of the ExEx is just an async function.
///
/// During initialization you can wait for resources you need to be up for the ExEx to function,
/// like a database connection.
async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(exex(ctx))
}

/// An ExEx is just a future, which means you can implement all of it in an async function!
///
/// This ExEx just prints out whenever either a new chain of blocks being added, or a chain of
/// blocks being re-orged. After processing the chain, emits an [ExExEvent::FinishedHeight] event.
async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>) -> eyre::Result<()> {
    while let Some(notification) = ctx.notifications.try_next().await? {
        match &notification {
            ExExNotification::ChainCommitted { new } => {
                info!(committed_chain = ?new.range(), "Received commit");
                let block = new.block(new.tip().block.hash()).unwrap();
                let block_number = block.number;
                let transaction_length = block.body.transactions.len() as u64;
                let mut client = DataProcessorClient::new();
                client.write(block_number);
                client.write(transaction_length);
                let (proof, vk) = client.prove("../program".into()).unwrap();
                client.verify(&proof, &vk).expect("failed to verify proof");
            }
            ExExNotification::ChainReorged { old, new } => {
                info!(from_chain = ?old.range(), to_chain = ?new.range(), "Received reorg");
            }
            ExExNotification::ChainReverted { old } => {
                info!(reverted_chain = ?old.range(), "Received revert");
            }
        };

        if let Some(committed_chain) = notification.committed_chain() {
            ctx.events
                .send(ExExEvent::FinishedHeight(committed_chain.tip().num_hash()))?;
        }
    }

    Ok(())
}

fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("Minimal", exex_init)
            .launch()
            .await?;

        handle.wait_for_node_exit().await
    })
}

#[cfg(test)]
mod tests {
    use std::pin::pin;

    use alloy_consensus::TxLegacy;
    use alloy_primitives::{Address, TxKind, U256};
    use alloy_sol_types::SolEvent;
    use reth::revm::db::BundleState;
    use reth_execution_types::{Chain, ExecutionOutcome};
    use reth_exex_test_utils::{test_exex_context, PollOnce};
    use reth_primitives::{
        Block, BlockBody, Header, Log, Receipt, Receipts, Transaction, TransactionSigned, TxType,
    };
    use reth_testing_utils::generators::sign_tx_with_random_key_pair;

    /// Given the address of a bridge contract and an event, construct a transaction signed with a
    /// random private key and a receipt for that transaction.
    fn construct_tx(to: Address) -> eyre::Result<TransactionSigned> {
        let tx = Transaction::Legacy(TxLegacy {
            to: TxKind::Call(to),
            ..Default::default()
        });

        Ok(sign_tx_with_random_key_pair(&mut rand::thread_rng(), tx))
    }

    #[tokio::test]
    async fn test_exex() -> eyre::Result<()> {
        // Initialize a test Execution Extension context with all dependencies
        let (ctx, mut handle) = test_exex_context().await?;

        // Initialize the Execution Extension
        let mut exex = pin!(super::exex_init(ctx).await?);

        // Generate random "from" and "to" addresses for deposit and withdrawal events
        let from_address = Address::new([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
        ]);
        let to_address = Address::random();

        let deposit_tx = construct_tx(to_address)?;
        let withdrawal_tx = construct_tx(from_address)?;

        let block = Block {
            header: Header::default(),
            body: BlockBody {
                transactions: vec![deposit_tx, withdrawal_tx],
                ..Default::default()
            },
        }
        .seal_slow()
        .seal_with_senders()
        .ok_or_else(|| eyre::eyre!("failed to recover senders"))?;
        // Construct a chain
        let chain = Chain::new(
            vec![block.clone()],
            ExecutionOutcome::new(
                BundleState::default(),
                Receipts::default(),
                block.number,
                vec![block.body.requests.clone().unwrap_or_default()],
            ),
            None,
        );

        // Send a notification that the chain has been committed
        handle
            .send_notification_chain_committed(chain.clone())
            .await?;
        // Poll the ExEx once, it will process the notification that we just sent
        exex.poll_once().await?;

        Ok(())
    }
}
