# Token Factory Example

This repo uses [terrain](https://github.com/terra-money/terrain) to do most of the work.

## Build and deploy

```
terrain deploy token-factory-example
```

## Create denom
```
terrain task:run create_token
```

## Mint tokens
Replace the hardcoded denom in the script (tasks/mint_token.ts) with the denom created from the previous step.
```
terrain task:run mint_token
```

## Burn tokens
Replace the hardcoded denom in the script (tasks/burn_token.ts) with the denom created from the previous step.

```
terrain task:run burn_token
```
