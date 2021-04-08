const scoket = new WebSocket("ws://localhost:8080/ws");

scoket.onopen = (event) => {
    scoket.send(JSON.stringify({ type: "join"}));
};

scoket.onerror = (event) => {
    console.log("Something wrong happened: " + JSON.stringify(event));
}

scoket.onmessage = (event) => {
    if (!event.data) return;

    const msg = JSON.parse(event.data);

    switch(msg.type) {
        case "user-connected":
            console.log("User connected!");
            break;
        case "user-disconnected":
            console.log("User disconnected!");
            break;
    }
}

// const myVideo = document.createElement('video')
// myVideo.muted = true
// const peers = {}
// navigator.mediaDevices.getUserMedia({
//   video: true,
//   audio: true
// }).then(stream => {
//   addVideoStream(myVideo, stream)

// //   socket.on('user-connected', userId => {
// //     connectToNewUser(userId, stream)
// //   })
// })

function addVideoStream(video, stream) {
    video.srcObject = stream
    video.addEventListener('loadedmetadata', () => {
      video.play()
    })
    const videoContainer = document.getElementById('video-container')
    videoContainer.append(video)
}