using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

public class Context : MonoBehaviour
{
    const string HOSTNAME = "localhost";
    const string PORT = "443"; // 443
    const string PROTOCOL = "https:"; // "https:"

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
