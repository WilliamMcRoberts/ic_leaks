<script>
  import CanisterIds from "./components/CanisterIds.svelte";
  import Links from "./components/Links.svelte";
  import logo from "./assets/logo.png";
  import {
    createActor,
    ic_leaks_backend,
  } from "../../declarations/ic_leaks_backend";
  import { AuthClient } from "@dfinity/auth-client";
  import { HttpAgent } from "@dfinity/agent";

  let actor = ic_leaks_backend;
  console.log(process.env.CANISTER_ID_INTERNET_IDENTITY);

  let whoAmIButton;
  let principalElement;
  let loginButton;

  let disableWhoAmIButton = false;

  async function handleWhoAmI(e) {
    e.preventDefault();
    disableWhoAmIButton = true;
    const principal = await actor.whoami();
    disableWhoAmIButton = false;
    principalElement.innerText = principal.toString();
    return false;
  }

  async function handleLogin(e) {
    e.preventDefault();
    let authClient = await AuthClient.create();
    // start the login process and wait for it to finish
    console.log("Auth Client: ", authClient);
    await new Promise((resolve) => {
      console.log("Starging Login");
      authClient.login({
        identityProvider:
          process.env.DFX_NETWORK === "ic"
            ? "https://identity.ic0.app"
            : "http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943",

        onSuccess: resolve,
      });
    });
    const identity = authClient.getIdentity();
    console.log("Identity: ", identity);
    const agent = new HttpAgent({ identity });
    console.log("Agent: ", agent);
    actor = createActor(process.env.CANISTER_ID_IC_LEAKS_BACKEND, {
      agent,
    });
    console.log("Actor: ", actor);
    return false;
  }
</script>

<main>
  <a
    href="https://internetcomputer.org"
    target="_blank"
    rel="noopener noreferrer"
    class="logo"
  >
    <img src={logo} alt="ICP logo" />
  </a>
  <h1>Svelte Starter dApp</h1>
  <h1>Svelte Change Title</h1>
  <Links />
  <CanisterIds />
  <form>
    <button bind:this={loginButton} on:click={handleLogin} id="login"
      >Login!</button
    >
  </form>
  <br />
  <img src={logo} alt="ICP logo" />
  <form>
    <button
      bind:this={whoAmIButton}
      on:click={handleWhoAmI}
      disabled={disableWhoAmIButton}
      id="whoAmI">Who Am I</button
    >
  </form>
  <section bind:this={principalElement} id="principal" />
</main>

<style>
  img {
    height: 22px;
  }
  .logo {
    display: inline-block;
    margin-bottom: 64px;
    margin-top: 24px;
  }

  main {
    text-align: center;
    padding: 1em;
  }

  h1 {
    text-transform: uppercase;
    font-size: 3em;
    font-weight: 400;
    line-height: 1.09;
  }

  @media (min-width: 640px) {
    main {
      max-width: 800px;
      margin: 0 auto;
    }
    h1 {
      font-size: 4em;
    }
  }
</style>
