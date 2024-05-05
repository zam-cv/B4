using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

/* 
 * Esta clase se encarga de guardar la información de la conexión
 * con el servidor y de mantenerla en toda la aplicación.
 */

public class Context : MonoBehaviour
{
    const string HOSTNAME = "localhost";
    const string PORT = "8080"; // 443
    const string PROTOCOL = "http:"; // "https:"

    public static Context Instance { get; private set; }
    public string AuthToken { get; set; }
    public string HostName { get; set; } = HOSTNAME;
    public string Port { get; set; } = PORT;
    public string Protocol { get; set; } = PROTOCOL;
    public string Host { get; set; } = HOSTNAME + ":" + PORT;
    public string ServerUrl { get; set; } = PROTOCOL + "//" + HOSTNAME + ":" + PORT + "/api";
    public string WebSocketUrl { get; set; } = "ws" + (PROTOCOL == "https:" ? "s" : "") + "://" + HOSTNAME + ":" + PORT + "/ws/";

    [DllImport("__Internal")]
    private static extern string GetHostname();

    // Esta función hace las configuraciones iniciales para el contexto del jugador
    // que será modificado y guardado en la base de datos al cerrar la conexión.
    void Start()
    {
        if (Application.platform == RuntimePlatform.WebGLPlayer)
        {
            string hostname = GetHostname();
            Instance.HostName = hostname;
            Instance.Host = hostname + ":" + Instance.Port;
            Instance.ServerUrl = Protocol + "//" + Instance.Host + "/api";
            Instance.WebSocketUrl = "ws" + (Protocol == "https:" ? "s" : "") + "://" + Instance.Host + "/ws/";
            Debug.Log("Hostname: " + hostname);
        }
        else
        {
            Debug.Log("Not WebGL");
        }

        Debug.Log("WebSocketUrl: " + Instance.WebSocketUrl);
    }
    
    // Verifica si ya existe una instancia de la clase Context y si no, la crea.
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
}
