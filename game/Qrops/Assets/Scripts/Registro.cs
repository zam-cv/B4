using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using UnityEngine.SceneManagement;
using TMPro;
using UnityEditor.Build.Content;
using UnityEditor.XR;
using System;

public class Registro : MonoBehaviour
{    
    [SerializeField] private TMP_InputField username;

    [SerializeField] private TMP_InputField password;

    [SerializeField] private TMP_InputField user_name;

    [SerializeField] private TMP_InputField email;

    [SerializeField] private TMP_InputField year_of_birth;

    [SerializeField] private TMP_Dropdown gender;
    private readonly string[] genderOptions = { "M", "F", "X" };

    [SerializeField] private TMP_Dropdown user_type;

    [System.Serializable]
    public struct UserData
    {
        public string username;
        public string password;
        public string user_name;
        public string email;
        public int year_of_birth;
        public string gender;
        public string user_type;
    }

public void Start(){
    username = GameObject.Find("Username_input").GetComponent<TMP_InputField>();
    password = GameObject.Find("Password_input").GetComponent<TMP_InputField>();
    user_name = GameObject.Find("Name_input").GetComponent<TMP_InputField>();
    email = GameObject.Find("Email_input").GetComponent<TMP_InputField>();
    year_of_birth = GameObject.Find("Age_input").GetComponent<TMP_InputField>();
    gender = GameObject.Find("Gender_input").GetComponent<TMP_Dropdown>();
    user_type = GameObject.Find("User_type_input").GetComponent<TMP_Dropdown>();

}
    public void Registration()
    {
        StartCoroutine(Register_request());
    }

    IEnumerator Register_request()
    {
        UserData user = new UserData();
        user.username = username.text.ToString();
        user.password = password.text.ToString();
        user.user_name = user_name.text.ToString();
        user.email = email.text.ToString();
        user.year_of_birth = Int32.Parse(year_of_birth.text);
        user.gender = genderOptions[gender.value];
        user.user_type = user_type.options[user_type.value].text;


        string json = JsonUtility.ToJson(user);
        print(json);

        UnityWebRequest request = UnityWebRequest.Post("http://localhost:8080/api/auth/register", json, "application/json");

        yield return request.SendWebRequest();

        if (request.result == UnityWebRequest.Result.Success)
        {
            string token = request.downloadHandler.text;
            // Context.Instance.AuthToken = token;
            SceneManager.LoadScene("Login");
        }
        else
        {
            print("ERROR: " + request.error);
        }
    }

    public void BackToLogin()
    {
        SceneManager.LoadScene("Login");
    }
}
