import { Env, task } from "@terra-money/terrain";

task(async (env:Env) => {
    try {
        let res = await env.client.execute(env.defaultWallet, "token-factory-example", {
            create_token: {
                denom: "utest",
                name: "Test Token",
                symbol: "TEST",
            }
        }, { "uluna": 10_000_000})
        console.log("NEW TOKEN DENOM", res.logs[0]["events"][2]["attributes"][1].value);
        console.log(JSON.stringify(res.logs, null, 2))
    } catch (e) {
        console.log(e);
    }
});
