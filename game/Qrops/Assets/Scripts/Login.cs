using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;
using TMPro;
/*
 * Clase que se encarga de manejar el login del jugador
 */
public class Login : MonoBehaviour
{
    [SerializeField] private TMP_InputField username;

    [SerializeField] private TMP_InputField password;

    [System.Serializable]
    public struct UserData
    {
        public string username;
        public string password;
    }
    // Fncion que se ejecuta al iniciar el juego
    // Se encarga de verificar si el jugador ya tiene un token de sesión
    // Si lo tiene, se intenta autenticar al jugador sin necesidad de que se loguee

    void Start()
    {
        // Para borrar el token de la sesión para probar login
        //PlayerPrefs.DeleteAll();

        if (PlayerPrefs.HasKey("token"))
        {
            AttemptAuth();
        }

        username = GameObject.Find("Username").GetComponent<TMP_InputField>();
        password = GameObject.Find("Password").GetComponent<TMP_InputField>();
        password.contentType = TMP_InputField.ContentType.Password;
    }


    // Funcion que intenta autenticar al jugador
    public void AttemptAuth()
    {
        StartCoroutine(RequestAuth());
    }

    // Funcion que realiza la petición de autenticación al servidor
    IEnumerator RequestAuth()
    {
        string token = PlayerPrefs.GetString("token");
        // UnityWebRequest request = UnityWebRequest.Get("http://localhost:8080/api/auth");
        UnityWebRequest request = UnityWebRequest.Get(Context.Instance.ServerUrl + "/auth");
        request.SetRequestHeader("token", token);

        yield return request.SendWebRequest();

        if (request.result == UnityWebRequest.Result.Success && request.responseCode == 200)
        {
            // Token es válido
            Debug.Log("Token validado con éxito: " + request.downloadHandler.text);
            Context.Instance.AuthToken = token;
            SceneManager.LoadScene("Game");
        }
        else
        {
            PlayerPrefs.DeleteKey("token");
            SceneManager.LoadScene("Login");
        }
    }
    
    // Funcion que se ejecuta al presionar el boton de login
    public void AttemptLogin()
    {
        StartCoroutine(RequestLogin());
    }

    // Funcion que realiza la petición de login al servidor
    IEnumerator RequestLogin()
    {
        UserData user = new UserData();
        user.username = username.text.ToString();
        user.password = password.text.ToString();

        string json = JsonUtility.ToJson(user);

        UnityWebRequest request = UnityWebRequest.Post(Context.Instance.ServerUrl + "/auth/signin", json, "application/json");

        yield return request.SendWebRequest();

        print(json);

        if (request.result == UnityWebRequest.Result.Success)
        {
            string token = request.downloadHandler.text;
            Context.Instance.AuthToken = token;

            PlayerPrefs.SetString("token", token);
            PlayerPrefs.Save();

            SceneManager.LoadScene("Game");

        }
        else
        {
            print("ERROR: " + request.error);
        }
    }

    // Funcion que se ejecuta al presionar el boton de registro
    public void GoToSignUp()
    {
        SceneManager.LoadScene("Registro");
    }
}
