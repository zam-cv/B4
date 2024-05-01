using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public struct CropData
{
    public string type;
    public string name;
    public int quantity;
    public string moneyType;
}

public class Shop : MonoBehaviour
{
    public static Shop Instance { get; private set; }

    public GameObject panel;
    public TMP_Text cropName;
    public TMP_Text cropPrice;
    public TMP_Text cropDuration;

    public TMP_InputField cropQuantity;

    public CropData cropData;

    void Start()
    {
        if (Instance == null)
        {
            Instance = this;
        }
    }

    private void Awake()
    {
        if (Instance == null)
        {
            cropData = new CropData();
            cropData.type = "BuyCrop";
            cropData.name = "tomate";
            cropData.quantity = 10;
            cropData.moneyType = "Cash";

            panel = GameObject.Find("Tienda");
            cropName = GameObject.Find("Nombre").GetComponent<TMP_Text>();
            cropPrice = GameObject.Find("Precio").GetComponent<TMP_Text>();
            cropDuration = GameObject.Find("Duracion").GetComponent<TMP_Text>();
            cropQuantity = GameObject.Find("Cantidad").GetComponent<TMP_InputField>();
            cropQuantity.text = cropData.quantity.ToString();
            panel.SetActive(false);

            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }

    public void SetCropType(CropType cropType)
    {
        cropName.text = cropType.name;
        cropPrice.text = cropType.price.ToString();
        cropDuration.text = cropType.duration.ToString();

        cropData.name = cropType.name;
    }

    public void SelectCropType(string cropType)
    {
        cropName.text = cropType;
        cropData.name = cropType;

        if (Utils.Instance.crops.TryGetValue(cropType, out CropType crop))
        {
            SetCropType(crop);
        }
    }

    public void SelectMaiz()
    {
        SelectCropType("maiz");
    }

    public void SelectTomate()
    {
        SelectCropType("tomate");
    }

    public void SelectCebada()
    {
        SelectCropType("cebada");
    }

    public void SelectCana()
    {
        SelectCropType("cana"); // caña
    }
}
