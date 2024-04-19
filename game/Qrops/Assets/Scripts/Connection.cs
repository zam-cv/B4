using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;

using NativeWebSocket;

public struct Player
{
    public float current_score;
    public int balance_verqor;
    public int balance_coyote;
    public int balance_cash;
}
public class Connection : MonoBehaviour
{
    WebSocket websocket;


    struct CycleResolved
    {
        public string type;
        public List<string> events;
        public Player player;
        public string tip;
    }


    void Start()
    {
        StartAsync();
    }

    async void StartAsync()
    {
        string token = Context.Instance.AuthToken;
        Debug.Log("Token: " + token);
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
            Debug.Log("Connection closed!" + e);
        };

        websocket.OnMessage += (bytes) =>
        {
            var message = System.Text.Encoding.UTF8.GetString(bytes);
            var data = JsonConvert.DeserializeObject<Dictionary<string, object>>(message);

            switch (data["type"].ToString())
            {
                case "Init":
                    Player player = JsonConvert.DeserializeObject<Player>(message);
                    
                    gameObject.GetComponent<State>().SetState(player);

                    break;
                case "CycleResolved":
                    Debug.Log(message);
                    CycleResolved cycleResolved = JsonConvert.DeserializeObject<CycleResolved>(message);
                    Debug.Log(cycleResolved.events[0]);
                    gameObject.GetComponent<State>().SetState(cycleResolved.player);


                    break;
                    // Add more cases here
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


    // Crear una funcion asincrona para hacer un ciclo
    // La funcion envia un json al socket y recibe otro json
    public async void HacerCiclo()
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"type", "Cycle"},
                {"duration", 12}
            };
            string jsonMessage = JsonConvert.SerializeObject(messageData);

            // Enviar el mensaje JSON
            await websocket.SendText(jsonMessage);
        }
        else
        {
            Debug.Log("WebSocket no está conectado.");
        }
    }

}
