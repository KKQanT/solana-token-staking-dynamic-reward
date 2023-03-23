# solana-token-staking-dynamic-reward
Solana token staking smart contract with dynamic reward depending on dividual proportion of token staked. 

This program has 4 primary state accounts that allow for the dynamic calculation of rewards based on the percentage of tokens contributed by stakers to the pool during each time interval (epoch). Just like the other staking programs, there are vault and pool account. Vault account is used to embed the staking info such as amount and time in which user start to stake. Pool account is used to be an authority account that hold the token that will be paid as reward to staker. However, in order to store the information used to calculate dynamic reward, I employed the epoch state accounts that used to stored


Dynamic reward formular

$$ reward_{i,t} = {\frac{{stakedToken}_{i,t}}{ sum_{i = 1}^{N} stakedToken_{i,t}} * allocated_reward_{i,t}}$$
