try{
    webSocket = new WebSocket("ws://127.0.0.1:8000/ws");
}catch(e){
    console.log(e);
}