using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

public class Context : MonoBehaviour
{
    const string HOSTNAME = "localhost";
    const string PORT = "8080";
    const string PROTOCOL = "http:";

    public static Context Instance { get; private set; }
    public string AuthToken { get; set; }
    public string HostName { get; set; } = HOSTNAME;
    public string Port { get; set; } = PORT;
    public string Protocol { get; set; } = PROTOCOL;
    public string Host { get; set; } = HOSTNAME + ":" + PORT;
    public string ServerUrl { get; set; } = PROTOCOL + "//" + HOSTNAME + ":" + PORT + "/api";

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
            Debug.Log("Hostname: " + hostname);
        }
        else
        {
            Debug.Log("Not WebGL");
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
}
