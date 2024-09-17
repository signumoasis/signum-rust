#[derive(Debug)]
pub struct Block;

impl From<ExchangeableBlock> for Block {
    fn from(value: ExchangeableBlock) -> Self {
        todo!()
    }
}

/// Maybe not needed at all
#[derive(Debug)]
pub struct ExchangeableBlock;

impl From<Block> for ExchangeableBlock {
    fn from(value: Block) -> Self {
        todo!()
    }
}
