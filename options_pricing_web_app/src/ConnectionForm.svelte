<script>
  import { getContext } from 'svelte';
  import { connectMachine } from './stores';
  import { createSolaceClient, solaceContextKey } from './solace-client';
  import { solaceConfig } from './solace.config';
  import ConnectionSpinner from './ConnectionSpinner.svelte';
  import { fly } from 'svelte/transition';

  const { state, send } = $connectMachine;

  const { getSolaceClient } = getContext(solaceContextKey);
  let solaceClient = getSolaceClient();

  let url = solaceConfig.SOLACE_HOST_URL;
  let vpnName = solaceConfig.SOLACE_MESSAGE_VPN;
  let userName = solaceConfig.SOLACE_USERNAME;
  let password = solaceConfig.SOLACE_PASSWORD;

  async function handleConnect() {
    let _solaceClient = createSolaceClient({
      url,
      vpnName,
      userName,
      password,
    });
    _solaceClient.setOnUpNotice(() => {
      send('UP_NOTICE');
    });
    _solaceClient.setOnConnectFailedError(() => {
      send('ERROR');
    });
    _solaceClient.setOnDisconnected(() => {
      send('DISCONNECTED');
    });

    send('CONNECT_REQUEST');
    $solaceClient = await _solaceClient.connect();
  }

  async function handleDisconnect() {
    if ($solaceClient) {
      $solaceClient.disconnect();
      $solaceClient = null;
    }
  }
</script>

<style>
  .flex-container {
    display: flex;
    justify-content: center;
    background-color: rgb(0, 56, 37);
  }
  .flex-item {
    padding: 5px;
    margin-top: 10px;
    text-align: center;
  }
  label {
    font-size: 12px;
    color: #00c895;
    font-weight: bold;
  }

  input {
    background-color: white;
    font-size: 12px;
    color: black;
    size: 20;
  }

  .button-connect {
    background-color: #00c895;
    color: white;
    font-weight: bold;
    cursor: pointer;
    height: 50px;
  }

  .button-disconnect {
    background-color: red;
    color: white;
    font-weight: bold;
    cursor: pointer;
    height: 35px;
  }
</style>

{#if $state.value != 'connected'}
  <div class="flex-container" in:fly={{ y: -100, duration: 2000 }} out:fly={{ y: -100, duration: 2000 }}>
    <div class="flex-item">
      <label for="brokerUrl">Broker URL</label>
      <div><input id="brokerUrl" placeholder="" bind:value={url} /></div>
    </div>
    <div class="flex-item">
      <label for="brokerVPN">VPN</label>
      <div><input id="brokerVPN" placeholder="" bind:value={vpnName} /></div>
    </div>
    <div class="flex-item">
      <label for="brokerUser">Username</label>
      <div><input id="brokerUser" placeholder="" bind:value={userName} /></div>
    </div>
    <div class="flex-item">
      <label for="brokerPassword">Password</label>
      <div><input id="brokerPassword" placeholder="" bind:value={password} /></div>
    </div>
    <div class="flex-item"><button on:click={handleConnect} type="button" class="button-connect"> Connect </button></div>
    {#if $state.value == 'connecting'}
      <ConnectionSpinner />
    {/if}
  </div>
{:else if $state.value == 'connected'}
  <div class="flex-container" in:fly={{ y: -200, duration: 2000 }} out:fly={{ y: -200, duration: 2000 }}>
    <div class="flex-item">
      <h4>Connected to Solace!</h4>
    </div>
  </div>
{/if}
