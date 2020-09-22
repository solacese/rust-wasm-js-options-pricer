<script>
  import { onMount, setContext } from 'svelte';
  import { writable } from 'svelte/store';
  import { solaceContextKey } from './solace-client';
  import ConnectionForm from './ConnectionForm.svelte';
  import OptionsTable from './OptionsTable.svelte';
  import wasm from '../../black_scholes_option_pricer/Cargo.toml';

  let black_scholes_pricer = null;

  let symbols = [];

  let optionMarketData = [];

  let solaceClient = writable(null);
  setContext(solaceContextKey, {
    getSolaceClient: () => solaceClient,
  });

  onMount(async () => {
    black_scholes_pricer = await wasm();
  });

  $: {
    if ($solaceClient) {
      $solaceClient.subscribe('OPTIONS/MARKETDATA/>', (msg) => {
        let optionData = JSON.parse(msg.getBinaryAttachment());
        console.log('Received ' + optionData.ticker + 'tick');
        optionMarketData[optionData.ticker] = optionData;
        if (!symbols.includes(optionData.ticker)) {
          symbols.push(optionData.ticker);
        }
      });
    }
  }
</script>

<style>
  :global(body) {
    background-color: black;
  }

  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    color: aquamarine;
    background-color: black;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }

  .flex-container {
    display: flex;
  }
  .flex-item {
    padding: 5px;
    margin-top: 10px;
    text-align: center;
  }
</style>

<main>
  <h1>Real-Time Web Assembly Options Pricer</h1>

  <ConnectionForm />
  {#if black_scholes_pricer}
    <div class="flex-container">
      {#each symbols as symbol}
        <div class="flex-item">
          <OptionsTable {black_scholes_pricer} optionMarketData={optionMarketData[symbol]} />
        </div>
      {/each}
    </div>
  {/if}
</main>
