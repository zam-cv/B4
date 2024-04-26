using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;
using UnityEngine.SceneManagement;
using NativeWebSocket;
using TMPro;

public struct Player
{
    public int time;
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
    public int growth;
}

public struct CropType
{
    public string name;
    public int price;
    public int duration;
    public string description;
}

public struct InitialData
{
    public List<Plot> plots;
    public List<string> top_players;
    public List<CropType> crops_types;
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

    public GameObject events_window;
    GameObject scoreObject;
    TMP_Text scoreObjectText;

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
        websocket = new WebSocket(Context.Instance.WebSocketUrl, headers);

        scoreObject = GameObject.Find("Lista");
        scoreObjectText = scoreObject.GetComponent<TMP_Text>();

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
                    ModifiedPlayer<InitialData> initData = JsonConvert.DeserializeObject<ModifiedPlayer<InitialData>>(message);
                    player = initData.player;
                    Utils.Instance.SetState(player);
                    Utils.Instance.SetTopPlayers(initData.payload.top_players);
                    Utils.Instance.SetCropsTypes(initData.payload.crops_types);
                    // set the crops with data.payload.crops_types
                    // set the plots with data.payload.plots
                    break;
                case "CycleResolved":
                    Debug.Log(message);
                    ModifiedPlayer<CycleResolved> cycleResolvedData = JsonConvert.DeserializeObject<ModifiedPlayer<CycleResolved>>(message);
                    player = cycleResolvedData.player;
                    Utils.Instance.SetState(player);
                    Debug.Log(cycleResolvedData.payload.events[0]);
                    // Funcion que modifica el contenido del scoreObjectText con los eventos
                    //scoreObjectText.text = cycleResolvedData.payload.events[0];
                    scoreObjectText.text = "";
                    foreach (string evento in cycleResolvedData.payload.events)
                    {
                        scoreObjectText.text += evento + "\n";
                    }
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

    public void Cycle1M()
    {
        Cycle("1M");
    }

    public void Cycle6M()
    {
        Cycle("6M");
    }

    public void Cycle1Y()
    {
        Cycle("1Y");
    }

    // Crear una funcion asincrona para hacer un ciclo
    // La funcion envia un json al socket y recibe otro json
    public async void Cycle(string duration)
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"type", "Cycle"},
                {"duration", duration} // 1M = 1 mes, 6M = 6 meses, 1Y = 1 año
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
