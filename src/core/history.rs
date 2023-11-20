use super::{AppendToHistoryError, Block};

pub struct History {
    chain: Vec<Block>,
    reorg_chain_strategy: Box<dyn ReorgChainStrategy>,
}

impl History {
    pub fn new(reorg_strategy: Box<dyn ReorgChainStrategy>) -> History {
        History {
            chain: vec![Block::genesis()],
            reorg_chain_strategy: reorg_strategy,
        }
    }

    pub fn try_to_append(&mut self, new_block: Block) -> Result<bool, AppendToHistoryError> {
        let tail_block = self.chain.last().ok_or(AppendToHistoryError {})?;

        new_block.verify(tail_block)?;

        self.chain.push(new_block);

        Ok(true)
    }

    pub fn choose_chain(&self, other_chain: &Vec<Block>) -> History {
        // todo: should verify other chain
        let chosen_chain = self
            .reorg_chain_strategy
            .chose_chain(&self.chain, other_chain);

        let new_chain = match chosen_chain {
            ReorgChoice::First => self.chain.clone(),
            ReorgChoice::Second => other_chain.clone(),
        };

        History {
            chain: new_chain,
            reorg_chain_strategy: self.reorg_chain_strategy.clone()
        }
    }

    pub fn get_height(&self) -> usize {
        self.chain.len()
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}

pub enum ReorgChoice {
    First,
    Second,
}

pub trait ReorgChainStrategy {
    fn chose_chain(&self, first_chain: &Vec<Block>, second_chain: &Vec<Block>) -> ReorgChoice;
    fn clone_dyn(&self) -> Box<dyn ReorgChainStrategy>;
}

#[derive(Clone)]
pub struct NaiveReorgStrategy;
impl ReorgChainStrategy for NaiveReorgStrategy {
    fn chose_chain(&self, first_chain: &Vec<Block>, second_chain: &Vec<Block>) -> ReorgChoice {
        if first_chain.len() > second_chain.len() {
            return ReorgChoice::First;
        }

        ReorgChoice::Second
    }

    fn clone_dyn(&self) -> Box<dyn ReorgChainStrategy> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn ReorgChainStrategy> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[cfg(test)]
mod history_tests {
    use crate::core::{NaiveReorgStrategy, Block};

    use super::History;

    #[test]
    fn history_chose_chain_returns_a_new_history_with_chain_chosen_by_naive_strategy() {
        let mut hs = History::new(Box::new(NaiveReorgStrategy {}));
        let mut hs2 = History::new(Box::new(NaiveReorgStrategy {}));

        for _ in 0..5 {
            hs.chain.push(Block::genesis());
            hs2.chain.push(Block::genesis());
        }
        for _ in 0..3 {
            hs2.chain.push(Block::genesis());
        }

        let new_hs = hs.choose_chain(&hs2.chain);
        
        assert_eq!(hs2.get_height(), new_hs.get_height());
    }
}