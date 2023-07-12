# <img src="https://cryptologos.cc/logos/solana-sol-logo.png?v=025" with="25" height="25"> solana-token-staking-dynamic-reward
Solana token staking smart contract (Can be used for both FT and NFT) with dynamic reward depending on dividual proportion of token staked. 

Dynamic reward formular

$$ reward_{(i,t)} = \dfrac{stakedToken_{i,t}}{\sum\limits_{i=1}^{n}(stakedToken_{i,t})}\cdot allocatedReward_{t}$$

where

- reward_i_t denoted claimable reward of vault ith at epoch t
- stakedToken_i_t denoted amount of token stored in vault ith at epoch t
- allocatedReward_t denoted amount of rewards distributed for staker at epoch t

## Usage

### Setup
---
For basic setup and deployment, I have written [here](https://github.com/KKQanT/solana-nft-staking-program-constant-reward/tree/master) in the setup section.