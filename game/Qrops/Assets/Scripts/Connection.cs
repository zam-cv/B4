using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;
using UnityEngine.SceneManagement;
using NativeWebSocket;

public struct Player
{
    public float current_score;
    public int balance_verqor;
    public int balance_coyote;
    public int balance_cash;
}

public struct CycleResolved
{
    public string type;
    public List<string> events;
    public Player player;
    public string tip;
}

public struct Plot
{
    public string crop_type_id;
    public int quantity;
}

public struct InititialData
{
    public List<Plot> plots;
    public List<string> top_players;
}

public struct ModifiedPlayer<T>
{
    public Player player;
    public T payload;
}

public class Connection : MonoBehaviour
{
    public static Connection Instance { get; set; }
    WebSocket websocket;

    public GameObject loading_logo, loading_background;

    // state
    public Player player = new Player();

    void Start()
    {
        if (websocket == null)
        {
            StartAsync(); // Solo inicia la conexión si no está ya activa
        }
    }

    private void Awake()
    {
        if (Instance == null)
        {
            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }

    async void StartAsync()
    {
        string token = Context.Instance.AuthToken;
        Debug.Log("Token: " + token);
        Dictionary<string, string> headers = new Dictionary<string, string>();
        headers.Add("token", token);
        websocket = new WebSocket("ws://" + Context.Instance.Host + "/ws/", headers);

        websocket.OnOpen += () =>
        {
            Debug.Log("Connection open!");
            loading_logo.SetActive(false);
            loading_background.SetActive(false);
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
            var sample = JsonConvert.DeserializeObject<Dictionary<string, object>>(message);

            switch (sample["type"].ToString())
            {
                case "Init":
                    ModifiedPlayer<InititialData> initData = JsonConvert.DeserializeObject<ModifiedPlayer<InititialData>>(message);
                    player = initData.player;
                    Utils.Instance.SetState(player);
                    // set the plots with data.payload
                    break;
                case "CycleResolved":
                    Debug.Log(message);
                    ModifiedPlayer<CycleResolved> cycleResolvedData = JsonConvert.DeserializeObject<ModifiedPlayer<CycleResolved>>(message);
                    player = cycleResolvedData.player;
                    Debug.Log(cycleResolvedData.payload.events[0]);
                    break;
                case "CropBought":
                    ModifiedPlayer<List<Plot>> cropBoughtData = JsonConvert.DeserializeObject<ModifiedPlayer<List<Plot>>>(message);
                    player = cropBoughtData.player;
                    // set the plots with data.payload
                    break;
                    // Add more cases here
            }
        };

        // waiting for messages
        await websocket.Connect();
    }

    void Update()
    {
#if !UNITY_WEBGL || UNITY_EDITOR
        websocket.DispatchMessageQueue();
#endif
    }

    private async void OnApplicationQuit()
    {
        await websocket.Close();
    }

    // Crear una funcion asincrona para hacer un ciclo
    // La funcion envia un json al socket y recibe otro json
    public async void Cycle()
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"type", "Cycle"},
                {"duration", "1M"} // 1M = 1 mes, 6M = 6 meses, 1Y = 1 año
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

    public async void BuyCrop(string name, int quantity, string moneyType)
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"name", name},
                {"quantity", quantity},
                {"moneyType", moneyType}
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
