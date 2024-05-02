using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class Utils : MonoBehaviour
{
    public Dictionary<string, CropType> crops = new Dictionary<string, CropType>();
    public static Utils Instance { get; private set; }
    public GameObject contenedorMaices;
    public Sprite[] etapasCrecimientoMaiz;
    public Sprite[] etapasCrecimientoTomate;
    public Sprite[] etapasCrecimientoCebada;
    public Sprite[] etapasCrecimientoCaña;
    private Sprite[] etapasCrecimiento;
    public int indiceEtapa;
    private SpriteRenderer spriteRenderer;
    public int contCult = 0;

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
            Instance = this;
            DontDestroyOnLoad(gameObject);
        }
        else
        {
            Destroy(gameObject);
        }
    }

    public void SetPlots(List<Plot> plots)
    {
        contCult = 0;
        foreach (Plot plot in plots)
        {
            if (plot.crop_type_id != null)
            {
                indiceEtapa = GetGrowth(plot);

                //switch crop_type_id
                switch (plot.crop_type_id)
                {
                    case "tomate":
                        contenedorMaices = GameObject.Find("tomates"+contCult);  //tomates[cont];
                        etapasCrecimiento = etapasCrecimientoTomate;
                        break;
                    case "cana":
                        contenedorMaices = GameObject.Find("Cañas"+contCult);
                        etapasCrecimiento = etapasCrecimientoCaña;
                        break;
                    case "maiz":
                        contenedorMaices = GameObject.Find("Maices"+contCult); //maices[cont];
                        etapasCrecimiento = etapasCrecimientoMaiz;
                        break;
                    case "cebada":
                        contenedorMaices = GameObject.Find("cebadas"+contCult);
                        etapasCrecimiento = etapasCrecimientoCebada;
                        break;
                }
                //Guarda el contenedorMaices en el Queue de CultivosPlantados
                GameObject.Find("Click" + contCult).GetComponent<SelectParcela>().planted = true;
                //CultivosPlantados.instance.queueCultivos.Enqueue(contenedorMaices);
                CultivosPlantados.instance.cultivos[contCult] = contenedorMaices;

                bool reset = false;
                // Recorre todos los hijos del contenedor
                foreach (Transform hijo in contenedorMaices.transform)
                {
                    //obten el spriteRenderer del hijo actual
                    spriteRenderer = hijo.GetComponent<SpriteRenderer>();
                    spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
                    if (indiceEtapa >= 3)
                    {
                        reset = true;
                        spriteRenderer.sprite = null;
                    }
                }
                if (reset)
                {
                    contCult--;
                    reset = false;
                }


                // // Recorre todos los hijos del contenedor
                // foreach (Transform hijo in contenedorMaices.transform)
                // {
                //     //obten el spriteRenderer del hijo actual
                //     spriteRenderer = hijo.GetComponent<SpriteRenderer>();
                //     spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
                // }

                    //quantity = plot.quantity;
                contCult++;
            }
        }
    }

    public void SetState(Player player)
    {
        GameObject scoreObject = GameObject.Find("score");
        TMP_Text scoreText = scoreObject.GetComponent<TMP_Text>();

        GameObject verqorObject = GameObject.Find("verqor");
        TMP_Text verqorText = verqorObject.GetComponent<TMP_Text>();

        GameObject coyoteObject = GameObject.Find("coyote");
        TMP_Text coyoteText = coyoteObject.GetComponent<TMP_Text>();

        GameObject cashObject = GameObject.Find("cash");
        TMP_Text cashText = cashObject.GetComponent<TMP_Text>();
        
        double score = Mathf.RoundToInt(player.current_score * 100.0f);
        if (score < -100) score = -100;
        if (score > 100) score = 100;

        scoreText.text =  score + "%";
        verqorText.text = player.balance_verqor.ToString();
        coyoteText.text = player.balance_coyote.ToString();
        cashText.text = player.balance_cash.ToString();
    }

    public void SetTopPlayers(List<string> topPlayers)
    {
        GameObject topPlayersObject = GameObject.Find("topPlayers");
        TMP_Text topPlayersText = topPlayersObject.GetComponent<TMP_Text>();

        string topPlayersString = "";
        int i = 1;
        foreach (string player in topPlayers)
        {
            topPlayersString += i++ + ". " + player.Trim() + "\n";
        }

        topPlayersText.text = topPlayersString;
    }

    public void SetCropsTypes(List<CropType> cropsTypes)
    {
        foreach (CropType cropType in cropsTypes)
        {
            crops.Add(cropType.name, cropType);
        }
    }

    public int GetGrowth(Plot plot)
    {
        CropType cropType = crops[plot.crop_type_id];
        var porcentage =  (float)plot.growth / cropType.duration;
        var index = Mathf.Floor(porcentage * 4);

        if (index == 4)
        {
            return 3;
        }

        return (int)index;
    }
}
