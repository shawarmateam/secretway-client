<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import SendBtn from './assets/send-icon.svg'
  import { invoke } from '@tauri-apps/api'

  let inputText = '';
  let isInputTextNull = true;
  let textToShow: string[] = [];
  let isError = false;

  function handleInputSend()
  {
    if (inputText.length > 0 && isInputTextNull)
    {
      isInputTextNull = false;
      console.log("LOG: isInputTextNull: "+isInputTextNull);
    } 
    else if (!isInputTextNull && inputText.length == 0)
    {
      isInputTextNull = true;
      console.log("LOG: isInputTextNull: "+isInputTextNull);
    }
  }

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

  async function handleClickSend()
  {
    const send_msg_responce = await invoke('send_msg', { inputText });
    console.log(send_msg_responce);
    
    textToShow = [...textToShow, inputText];
    console.log(textToShow);
    inputText = '';
  }
  
</script>

<main>
  <div class="msgs">
    {#each textToShow as colToShow}
      <p>{colToShow}</p>
    {/each}
  </div>

  <div class="div-send">
    <input type="text" id={isError ? 'error-msg' : 'send-msg'}
      bind:value={inputText} on:keydown={handleKeyDown} on:input={handleInputSend} placeholder="Введите сообщение" />

    <button class="send-btn" on:click={handleClickSend} id={isInputTextNull ? 'hide-send-btn' : 'show-send-btn'}>
      <img src={SendBtn} alt="" width="20" height="20" />
    </button>
  </div>
</main>

<style>
  input[type=text]#send-msg {
    /* position: absolute;
    bottom: 10px;
    left: 10px; */

    height: 25px;
    /* width: calc(98% - 30px); */
    box-shadow: 0px 0px 10px rgba(0,0,0,0.7);
    background-color: #d5d5d5;
    color: #000;
    border-radius: 10px;
    width: calc(98% - 30px);

    margin-right: 10px;
    margin-left: 5px;
    transition:
      box-shadow .5s,
      transform .5s,
      outline .5s,
      border-color .5s;
  }

  #show-send-btn {
    margin-right: 5px;
  }

  #hide-send-btn {
    margin-right: -50px;
  }

  input[type=text]#send-msg:focus {
    /* border-color: #fff; */
    box-shadow: 0px 0px 10px rgba(255,255,255,0.7);
  }


  input[type=text]#error-msg {
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

  .send-btn {
    /* position: absolute;
    bottom: 8px;
    right: 10px; */

    width: 25px;
    height: 34px;
    background-color: #d5d5d5;
    box-shadow: 0px 0px 10px rgba(0,0,0,0.7);
    outline: none;
    border-radius: 10px;

    display: flex;
    align-items: center;
    justify-content: center;

    margin-bottom: 5px;

    transition:
      box-shadow .5s;
      background-color: .5s;
      margin-right: .5s;
  }

  .send-btn:hover {
    box-shadow: 0px 0px 10px rgba(255,255,255,0.7);
    background-color: white;
  }

  .div-send {
    display: flex;

    position: absolute;
    left: 0px;
    bottom: 0px;

    width: 100%;
    overflow: hidden;
  }
</style>
