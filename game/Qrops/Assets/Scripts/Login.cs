using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;
using TMPro;

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



    public void AttemptAuth()
    {
        StartCoroutine(RequestAuth());
    }

    IEnumerator RequestAuth()
    {
        string token = PlayerPrefs.GetString("token");
        UnityWebRequest request = UnityWebRequest.Get("http://localhost:8080/api/auth");
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

    public void AttemptLogin()
    {
        StartCoroutine(RequestLogin());
    }

    IEnumerator RequestLogin()
    {
        UserData user = new UserData();
        user.username = username.text.ToString();
        user.password = password.text.ToString();

        string json = JsonUtility.ToJson(user);

        UnityWebRequest request = UnityWebRequest.Post("http://localhost:8080/api/auth/signin", json, "application/json");

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

    public void GoToSignUp()
    {
        SceneManager.LoadScene("Registro");
    }
}
