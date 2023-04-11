import { Env, task } from "@terra-money/terrain";

task(async (env:Env) => {
  try {
    let res = await env.client.execute(env.defaultWallet, "token-factory-example", {
      burn_token: {}
    }, {"factory/terra1eyfccmjm6732k7wp4p6gdjwhxjwsvje44j0hfx8nkgrm8fs7vqfsrv3l3w/utest": 500000})
    console.log(res);
  }
    catch (e) {
        console.log(e);
  }
});
