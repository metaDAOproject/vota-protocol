# Vote Market Management Design
Design for scripts run each epoch to operate the vote market 

## Overview
The management commands will be added to the ./off-chain/vote-marken-manager
crate as new CLI subcommands. Each command will be in a file under ./src/manager.
The operator will need the script_authority private key to run the commands.
The operator will need to run several commands each epoch to 
operate the vote market. At first, this can be done manually, so
the results can be monitored.

## Commands
`calculate-inputs` This will produce a json file, useful for auditing,
with inputs to be used in other steps. It will find the following.
```text
b_i - the number of bribes for each gauge in USD value
a_i - the number of algorithmic votes for each gauge
d_i - the number of direct votes for each gauge
B - the sum of all b_i
A - the sum of all a_i
D - the sum of all d_i
The value of the vote lock/emissions token (SBR) in USD value
The amount of expected SBR emissions for the epoch being voted for
The expected emissions per vote in USD value
efficiency_ratio - the ratio of emission value to bribes
```
The amounts with the i subscript will be stored in a map with index of the gauge address.

`find-weights` - This will accept the output of `calculate-inputs` as
and input and produce a json file with key value pairs
of gauge address and weight.

`find-max-vote-buy` - This will compare E = (A + D) * emissions_per_vote with B. It will find 
MAX_VOTE_BUY = min(E/efficiency_ratio,B), then set the VoteBuy::max_amount to VoteBuy::amount * MAX_VOTE_BUY/B.

`execute-votes` - This will accept the output of `find-weights` as an input then 
vote for each delegated account for each gauge. It will use the same weights for each user.
For each user:
1) Set weight in a vote account for each gauge
2) Create an epoch gauge voter
3) commit votes by creating epoch gauge votes for each gauge

`upload-data` - This take the output of `calculate-inputs` and send data used to display the historical performance and APR in the UI.
```typescript
type EpochStats = {epoch: number,
    votePayment: number,
    votes: number,
    expectedSbrEmissions: number,
    sbrPrice: number,
    gauges: [{
      gauge: PublicKey,
      votePayment: number,
      votes: number
    }]
}
```

`airdrop-rewards` - For a given epoch, this will iterate through every escrow delegated to the vote market and
attempt to claim rewards for every gauge.
