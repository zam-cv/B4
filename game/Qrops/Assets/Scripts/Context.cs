using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

public class Context : MonoBehaviour
{
    public static Context Instance { get; private set; }
    public string AuthToken { get; set; }
    public string HostName { get; set; } = "localhost";
    public string Port { get; set; } = "8080";
    public string Host { get; set; } = "localhost:8080";
    public string ServerUrl { get; set; } = "http://localhost:8080/api";

    [DllImport("__Internal")]
    private static extern string GetHostname();

    void Start()
    {
        if (Application.platform == RuntimePlatform.WebGLPlayer)
        {
            string hostname = GetHostname();
            Instance.HostName = hostname;
            Instance.Host = hostname + ":" + Instance.Port + "/api";
            Instance.ServerUrl = "http://" + Instance.Host;
            Debug.Log("Hostname: " + hostname);
        } else
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
