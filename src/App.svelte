<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import { invoke } from '@tauri-apps/api'

  let inputText = '';
  let textToShow: string[] = [];
  let isError = false;

  async function handleKeyDown(event: KeyboardEvent) 
  {
    if (event.key === 'Enter' && inputText.length > 0) 
    {
        const send_msg_responce = await invoke('send_msg', { inputText });
        console.log(send_msg_responce);
        
        textToShow = [...textToShow, inputText];
        console.log(textToShow);
        inputText = '';
    } else if (!isError && event.key === 'Enter') {
      console.log("Error catched: Cannot send msg, when input field is null.");
      isError = true;

      setTimeout(() => {
        isError = false;
      }, 10);
    }
  }

  
</script>

<main>
  <div class="msgs">
    {#each textToShow as colToShow}
      <p>{colToShow}</p>
    {/each}
  </div>
  <input type="text" id={isError ? 'error-msg' : 'send-msg'} 
  bind:value={inputText} on:keydown={handleKeyDown} placeholder="Введите сообщение" />
</main>

<style>
  input[type=text]#send-msg {
    position: absolute;
    bottom: 10px;
    left: 10px;

    height: 25px;
    width: 95%;
    box-shadow: 0px 0px 10px rgba(0,0,0,0.7);
    background-color: #d5d5d5;
    color: #000;
    border-radius: 10px;
    transition:
      box-shadow .5s,
      transform .5s,
      outline .5s,
      border-color .5s;
  }

  input[type=text]#send-msg:focus {
    /* border-color: #fff; */
    box-shadow: 0px 0px 10px rgba(255,255,255,0.7);
  }


  input[type=text]#error-msg {
    position: absolute;
    bottom: 10px;
    left: 10px;

    height: 25px;
    width: 95%;
    background-color: #d5d5d5;
    color: #000;
    border-radius: 10px;

    border-color: #ff0000;
    box-shadow: 0px 0px 10px rgba(255, 0, 0, 0.7);
    transform: translateX(-10px);
  }

  input[type=text]#error-msg:focus {
    border-color: #ff0000;
    outline: 0;
  }

  div.msgs {
    position: absolute;
    top: 10px;
    left: 10px;

    text-align: left;
    overflow-y: auto;
    width: 98%;
    height: calc(100% - 55px);
    overflow-wrap: break-word;
  }

  div.msgs>p {
    margin: 0;
  }
</style>
