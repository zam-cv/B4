using System;
using System.Collections;
using System.Collections.Generic;
using JetBrains.Annotations;
using UnityEditor;
using UnityEngine;
using UnityEngine.Networking;
using Newtonsoft.Json;
using UnityEngine.SceneManagement;
using TMPro;
public class Select_crop : MonoBehaviour
{
    public TMP_Text balance;
    public TMP_Text crop_name;
    public TMP_Text crop_price;
    public TMP_Text crop_duration;

    public struct CropData
    {
        public string name;
        public string price;
        public string duration;
    }

     public string cropType;

    // Start is called before the first frame update
    void Start()
    {
        balance = GameObject.Find("Balance").GetComponent<TMP_Text>();
        crop_name = GameObject.Find("Nombre").GetComponent<TMP_Text>();
        crop_name.text = "";
        crop_price = GameObject.Find("Precio").GetComponent<TMP_Text>();
        crop_price.text = "";
        crop_duration = GameObject.Find("Duracion").GetComponent<TMP_Text>();
        crop_duration.text = "";
    }

    void AttemptCropRequest()
    {
        StartCoroutine(RequestCropType());
    }

    public void View_crop(GameObject crop)
    {
        AttemptCropRequest();
    }

    public void View_tomate()
    {
        cropType = "tomate";
    }
    public void View_cana()
    {
        cropType = "cana";
    }
    public void View_maiz()
    {
        cropType = "maiz";
    }
    public void View_cebada()
    {
        cropType = "cebada";
    }

    IEnumerator RequestCropType()
    {
        string token = Context.Instance.AuthToken;
        UnityWebRequest request = UnityWebRequest.Get(Context.Instance.ServerUrl + "/admin/data/crops/" + cropType);
        request.SetRequestHeader("token", token);

        yield return request.SendWebRequest();
        
        if (request.result == UnityWebRequest.Result.Success && request.responseCode == 200)
        {
            // Token es válido
            Debug.Log(request.downloadHandler.text);
            CropData cropData = JsonConvert.DeserializeObject<CropData>(request.downloadHandler.text);
            Debug.Log("Crop data: " + cropData.name + " " + cropData.price + " " + cropData.duration);
            crop_name.text = cropData.name;
            crop_price.text = "$ " + cropData.price;
            crop_duration.text = cropData.duration;
        }
    }

    IEnumerator RequestBalance()
    {
        string token = Context.Instance.AuthToken;
        UnityWebRequest request = UnityWebRequest.Get("http://localhost:8080/api/admin/data/balance");
        request.SetRequestHeader("token", token);

        yield return request.SendWebRequest();
        
        if (request.result == UnityWebRequest.Result.Success && request.responseCode == 200)
        {
            // Token es válido
            Debug.Log(request.downloadHandler.text);
            balance.text = request.downloadHandler.text;
        }
    }

    public void BackToGame(){
        //load the game scene
        SceneManager.LoadScene("Game");
    }
}