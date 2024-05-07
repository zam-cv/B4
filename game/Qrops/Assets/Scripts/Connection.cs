using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Newtonsoft.Json;
using UnityEngine.SceneManagement;
using NativeWebSocket;
using TMPro;

/* 
 * Esta clase se encarga de manejar la conexión con el servidor y de enviar y recibir mensajes
 * a través de un WebSocket. También se encarga de manejar los mensajes recibidos y de actualizar
 * el estado del juego en función de los mensajes recibidos.
 */

public struct Interest
{
    public int interest_verqor;
    public int interest_coyote;
}

public struct Message
{
    public string status;
    public string message;
}

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
    public List<Plot> plots;
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

public struct Harvested
{
    public Player player;
    public List<Plot> plots;
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

    public GameObject eventsPanel;
    public GameObject panelCiclos;

    public GameObject toast;
    public GameObject interestPanel;

    public GameObject loading_logo, loading_background;

    public GameObject events_window;
    GameObject scoreObject;
    TMP_Text scoreObjectText;

    // state
    public Player player = new Player();
    private SpriteRenderer spriteRenderer;

    // Inicia la conexión bajo la condición definida 
    void Start()
    {
        if (websocket == null)
        {
            StartAsync(); // Solo inicia la conexión si no está ya activa
        }
    }

    // Esta funcion se encarga ver si ya existe una instancia de la clase
    // y de no destruir el objeto al cambiar de escena
    private void Awake()
    {
        if (Instance == null)
        {
            panelCiclos = GameObject.Find("Elegir ciclos");
            eventsPanel = GameObject.Find("Eventos");
            scoreObject = GameObject.Find("Lista");
            scoreObjectText = scoreObject.GetComponent<TMP_Text>();
            eventsPanel.SetActive(false);
            panelCiclos.SetActive(false);
            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }
    
    // Esta funcion se encarga de iniciar la conexión con el servidor
    // y de escuchar los mensajes que envía el servidor
    async void StartAsync()
    {
        string token = Context.Instance.AuthToken;
        Debug.Log("Token: " + token);
        Dictionary<string, string> headers = new Dictionary<string, string>();
        headers.Add("token", token);
        websocket = new WebSocket(Context.Instance.WebSocketUrl, headers);

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
                    Utils.Instance.SetPlots(initData.payload.plots);
                    Utils.Instance.flagFirtsTime = false;

                    if (initData.payload.crops_types.Count > 0)
                    {
                        CropType cropType = initData.payload.crops_types[0];
                        Shop.Instance.SetCropType(cropType);
                    }
                    break;
                case "CycleResolved":
                    eventsPanel.SetActive(true);
                    panelCiclos.SetActive(false);

                    Debug.Log(message);
                    ModifiedPlayer<CycleResolved> cycleResolvedData = JsonConvert.DeserializeObject<ModifiedPlayer<CycleResolved>>(message);
                    player = cycleResolvedData.player;
                    Utils.Instance.SetState(player);
                    Debug.Log(cycleResolvedData.payload.events[0]);

                    // Utils.Instance.DeletePlots();
                    Utils.Instance.SetPlots(cycleResolvedData.payload.plots);

                    scoreObjectText.text = "";

                    foreach (string evento in cycleResolvedData.payload.events)
                    {
                        scoreObjectText.text += evento + "\n";
                    }
                    break;
                case "CropBought":
                    ModifiedPlayer<List<Plot>> cropBoughtData = JsonConvert.DeserializeObject<ModifiedPlayer<List<Plot>>>(message);
                    player = cropBoughtData.player;
                    Utils.Instance.SetState(player);
                    Utils.Instance.SetPlots(cropBoughtData.payload);
                    break;
                case "Message":
                    Message ObjMessage = JsonConvert.DeserializeObject<Message>(message);
                    GameObject toastClone = Instantiate(toast);

                    TMP_Text toastText = toastClone.transform.Find("Message").GetComponent<TMP_Text>();
                    toastText.text = ObjMessage.message;

                    UnityEngine.UI.Image statusImage = toastClone.transform.Find("Status").GetComponent<UnityEngine.UI.Image>();

                    if (ObjMessage.status == "Info")
                    {
                        statusImage.color = new Color(0.0f, 0.0f, 1.0f, 1.0f);
                    }
                    else if (ObjMessage.status == "Success")
                    {
                        statusImage.color = new Color(0.0f, 1.0f, 0.0f, 1.0f);
                    }
                    else if (ObjMessage.status == "Warning")
                    {
                        statusImage.color = new Color(1.0f, 1.0f, 0.0f, 1.0f);
                    }

                    toastClone.transform.SetParent(GameObject.Find("HUD").transform, false);
                    Destroy(toastClone, 3);
                    break;
                case "PlayerReseted":
                    ModifiedPlayer<List<Plot>> playerResetedData = JsonConvert.DeserializeObject<ModifiedPlayer<List<Plot>>>(message);
                    player = playerResetedData.player;
                    Utils.Instance.SetState(player);
                    Utils.Instance.SetPlots(playerResetedData.payload);
                    break;
                case "Interest":
                    Debug.Log(message);
                    Interest interest = JsonConvert.DeserializeObject<Interest>(message);
                    GameObject interestClone = Instantiate(interestPanel);
                    TMP_Text interestVerqor = interestClone.transform.Find("InterestVerqor").GetComponent<TMP_Text>();
                    TMP_Text interestCoyote = interestClone.transform.Find("InterestCoyote").GetComponent<TMP_Text>();
                    interestVerqor.text = interest.interest_verqor.ToString();
                    interestCoyote.text = interest.interest_coyote.ToString();
                    break;
                case "Harvested":
                    Debug.Log("Harvested" + message);
                    Harvested harvested = JsonConvert.DeserializeObject<Harvested>(message);
                    List<Plot> plots = harvested.plots;
                    player = harvested.player;
                    Utils.Instance.SetState(player);
                    Utils.Instance.DeletePlots();
                    Utils.Instance.SetPlots(plots);
                    break;
                    // Add more cases here
            }
        };

        // waiting for messages
        await websocket.Connect();
    }
    
    // Esta funcion se encarga de enviar un mensaje al servidor
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
    
    // Crear una funcion asincrona para comprar un cultivo
    public async void BuyCrop(string name, int quantity, string moneyType)
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"type", "BuyCrop"},
                {"name", name},
                {"quantity", quantity},
                {"money_type", moneyType}
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
    
    // Crear una funcion asincrona para comprar un cultivo con dinero en efectivo
    public void BuyCropCash()
    {
        if (int.TryParse(Shop.Instance.cropQuantity.text, out int quantity) && quantity > 0)
        {
            BuyCrop(Shop.Instance.cropData.name, quantity, "Cash");
        }
        else
        {
            Debug.Log("La cantidad debe ser un número mayor a 0");
        }
    }

    // Crear una funcion asincrona para comprar un cultivo con Verqor
    public void BuyCropVerqor()
    {
        if (int.TryParse(Shop.Instance.cropQuantity.text, out int quantity) && quantity > 0)
        {
            BuyCrop(Shop.Instance.cropData.name, quantity, "Verqor");
        }
        else
        {
            Debug.Log("La cantidad debe ser un número mayor a 0");
        }
    }
    
    // Crear una funcion asincrona para comprar un cultivo con Coyote
    public void BuyCropCoyote()
    {
        if (int.TryParse(Shop.Instance.cropQuantity.text, out int quantity) && quantity > 0)
        {
            BuyCrop(Shop.Instance.cropData.name, quantity, "Coyote");
        }
        else
        {
            Debug.Log("La cantidad debe ser un número mayor a 0");
        }
    }

    // Reinicia los valores del jugador
    public async void ResetPlayer()
    {
        // Verifica que la conexión esté abierta antes de enviar el mensaje
        if (websocket.State == WebSocketState.Open)
        {
            Utils.Instance.DeletePlots();

            // Crear el mensaje JSON para enviar
            var messageData = new Dictionary<string, object>
            {
                {"type", "ResetPlayer"}
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
