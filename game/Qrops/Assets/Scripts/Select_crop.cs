using System.Collections;
using System.Collections.Generic;
using JetBrains.Annotations;
using UnityEditor;
using UnityEngine;
using UnityEngine.Networking;

public class Select_crop : MonoBehaviour
{
    //Reference to info panel
    public GameObject info_panel;
    public GameObject tomate;
    public GameObject cana;
    public GameObject maiz;
    public GameObject cebada;
    public GameObject crop;

    [SerializeField] private string cropType;

    // Start is called before the first frame update
    void Start()
    {
        tomate = GameObject.Find("Anim_tomato");
        cana = GameObject.Find("Anim_cana");
        maiz = GameObject.Find("Anim_maiz");
        cebada = GameObject.Find("Anim_cebada");
        crop = new GameObject();

        cropType = "tomate";

        info_panel = GameObject.Find("Info_panel");
        Hide_all();
    }

    // Update is called once per frame
    void Update()
    {
        
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
    }

    public void View_tomate()
    {
        crop = tomate;
        View_crop(crop);
    }
    public void View_cana()
    {
        View_crop(cana);
    }

    IEnumerator RequesCropType()
    {
        string token = Context.Instance.AuthToken;
        UnityWebRequest request = UnityWebRequest.Get("http://localhost:8080/api/admin/data/crops/" + cropType);
        request.SetRequestHeader("token", token);

        yield return request.SendWebRequest();
        
        if (request.result == UnityWebRequest.Result.Success && request.responseCode == 200)
        {
            // Token es válido
            Debug.Log(request.downloadHandler.text);
        }

    }
}