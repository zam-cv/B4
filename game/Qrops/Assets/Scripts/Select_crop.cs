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
    public GameObject info_panel;
    public GameObject tomate;
    public GameObject cana;
    public GameObject maiz;
    public GameObject cebada;
    public GameObject crop;
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
        crop_name = GameObject.Find("Crop_name").GetComponent<TMP_Text>();
        crop_price = GameObject.Find("Crop_price").GetComponent<TMP_Text>();
        crop_duration = GameObject.Find("Crop_duration").GetComponent<TMP_Text>();

        tomate = GameObject.Find("Anim_tomato");
        cana = GameObject.Find("Anim_cana");
        maiz = GameObject.Find("Anim_maiz");
        cebada = GameObject.Find("Anim_cebada");
        crop = new GameObject();

        info_panel = GameObject.Find("Info_panel");
        Hide_all();
    }

    void AttemptCropRequest()
    {
        StartCoroutine(RequestCropType());
    }

    public void Show_info()
    { 
        info_panel.SetActive(true);
    }

    public void Hide_all()
    {
        info_panel.SetActive(false);
        tomate.SetActive(false);
        cana.SetActive(false);
        maiz.SetActive(false);
        cebada.SetActive(false);
    }

    public void View_crop(GameObject crop)
    {
        Hide_all();
        crop.SetActive(true);
        Show_info();
        AttemptCropRequest();
    }

    public void View_tomate()
    {
        cropType = "tomate";
        View_crop(tomate);
    }
    public void View_cana()
    {
        cropType = "cana";
        View_crop(cana);
    }
    public void View_maiz()
    {
        cropType = "maiz";
        View_crop(maiz);
    }
    public void View_cebada()
    {
        cropType = "cebada";
        View_crop(cebada);
    }

    IEnumerator RequestCropType()
    {
        string token = Context.Instance.AuthToken;
        UnityWebRequest request = UnityWebRequest.Get("http://localhost:8080/api/admin/data/crops/" + cropType);
        request.SetRequestHeader("token", token);

        yield return request.SendWebRequest();
        
        if (request.result == UnityWebRequest.Result.Success && request.responseCode == 200)
        {
            // Token es válido
            Debug.Log(request.downloadHandler.text);
            CropData cropData = JsonConvert.DeserializeObject<CropData>(request.downloadHandler.text);
            Debug.Log("Crop data: " + cropData.name + " " + cropData.price + " " + cropData.duration);
            crop_name.text = cropData.name;
            crop_price.text = cropData.price;
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