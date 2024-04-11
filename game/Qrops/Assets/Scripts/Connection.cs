using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;

using NativeWebSocket;

public class Connection : MonoBehaviour
{
    WebSocket websocket;

    async void Start()
    {
        string token = Context.Instance.AuthToken;
        Dictionary<string, string> headers = new Dictionary<string, string>();
        headers.Add("token", token);
        websocket = new WebSocket("ws://localhost:8080/ws/", headers);

        websocket.OnOpen += () =>
        {
            Debug.Log("Connection open!");
        };

        websocket.OnError += (e) =>
        {
            Debug.Log("Error! " + e);
        };

        websocket.OnClose += (e) =>
        {
            Debug.Log("Connection closed!");
        };

        websocket.OnMessage += (bytes) =>
        {
            var message = System.Text.Encoding.UTF8.GetString(bytes);
            var data = JsonConvert.DeserializeObject<Dictionary<string, object>>(message);

            switch (data["type"].ToString())
            {
                case "Init":
                    State.Instance.scoreText.text = data["current_score"].ToString();
                    State.Instance.cashText.text = data["balance_cash"].ToString();
                    State.Instance.verqorText.text = data["balance_verqor"].ToString();
                    State.Instance.coyoteText.text = data["balance_coyote"].ToString();
                    break;
                    // Add more cases hereb
            }
        };

        // Keep sending messages at every 0.3s
        InvokeRepeating("SendWebSocketMessage", 0.0f, 0.3f);

        // waiting for messages
        await websocket.Connect();
    }

    void Update()
    {
#if !UNITY_WEBGL || UNITY_EDITOR
        websocket.DispatchMessageQueue();
#endif
    }

    async void SendWebSocketMessage()
    {
        if (websocket.State == WebSocketState.Open)
        {
            // Sending bytes
            await websocket.Send(new byte[] { 10, 20, 30 });

            // Sending plain text
            await websocket.SendText("plain text message");
        }
    }

    private async void OnApplicationQuit()
    {
        await websocket.Close();
    }
}
