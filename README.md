# Superteam_Rust_Bootcamp_RewardSystem
Overview
----------
This project is a smart contract written in Rust, designed to create a reward system on the Solana blockchain. The smart contract allows users to mint reward tokens, distribute rewards, stake tokens, and earn rewards for staked tokens. It is written using the Anchor framework.

Features and Functions of the Program
----------
Token Minting

•initialize(): This function creates a new token mint (generation address) and mints a specified amount of tokens initially. These tokens are held in an account that represents the reward pool. For example, you can mint 1 million tokens to create a reward pool.

•Parameters:

  -amount (u64): The amount of tokens to be minted for the initial supply.

Reward Distribution

•distribute_reward(): This function distributes reward tokens to a user. A specified amount of tokens is transferred from the reward pool to the user's account. For example, you can give a user 100 tokens as a reward.

•Parameters:

  -amount (u64): The amount of tokens to be transferred.

Loan Actions

•loan_action(): This function allows users to earn reward tokens for performing certain actions, such as taking out a loan. The reward amount will be 10% of the loan amount. For example, if a loan of 1000 units is taken, 100 tokens will be awarded as a reward.

•Parameters:

  -loan_amount (u64): The amount of the loan.

Staking Tokens

•stake_tokens(): Users can stake the tokens they own. The staked tokens are held in the user's staking account and can be used later to earn additional rewards.

•Parameters:

  -stake_amount (u64): The amount of tokens to be staked.

Staking Rewards Distribution

•distribute_staking_rewards(): This function rewards users based on the tokens they have staked. An additional 10% of the staked amount is minted and added to the user's account as reward tokens. For example, if a user has staked 500 tokens, they will receive 50 reward tokens.

User Account Creation

•This function creates a new user account and initially sets staked_amount to zero.

Helper Functions
----------
•mint_tokens Function: Mints a specified amount of tokens to a given account.

  -Usage: This is used by other functions when minting tokens is required.

•transfer_tokens Function: Transfers tokens from one account to another.

  -Usage: This is used by other functions when a token transfer is necessary.

Configurations
----------
•Mint: The mint account used for token minting operations.

•Reward Account: The account where the tokens for rewards are held.

•User Account: The account where the user's staked tokens are stored.

Error Codes
----------
•InvalidAmount: Error indicating an invalid token amount.

•MintingFailed: Error indicating a failed token minting operation.

•TransferFailed: Error indicating a failed token transfer.

•NoStakedAmount: Error indicating zero staked token amount.

•Overflow: Error indicating a mathematical overflow.

Setup
----------
1-Requirements:

•Solana CLI

•Anchor framework

•Rust programming language

2-Clone and Build the Project:

git clone <repository_url>

cd <repository_directory>

anchor build

3-Connect to the Solana Network:

solana config set --url <network_url>

4-Deploy the Program:

anchor deploy

Purpose and Usage
----------
The purpose of this code is to create a reward system on the Solana network, allowing users to earn reward tokens for loan transactions and staking their tokens. It includes all the necessary operations for setting up the system, distributing rewards, and managing staking rewards. This system can be used in DeFi projects, games, or any application requiring a reward mechanism. You can scale your reward system by leveraging Solana’s fast and low-cost transactions.
