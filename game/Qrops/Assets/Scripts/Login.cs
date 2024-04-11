using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;
using TMPro;

public class Login : MonoBehaviour
{
    [SerializeField]
    private TMP_InputField username;

    [SerializeField]
    private TMP_InputField password;

    [System.Serializable]
    public struct UserData
    {
        public string username;
        public string password;
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

        if (request.result == UnityWebRequest.Result.Success)
        {
            string token = request.downloadHandler.text;
            Context.Instance.AuthToken = token;
            SceneManager.LoadScene("Game");
        }
        else
        {
            print("ERROR: " + request.error);
        }
    }
}
