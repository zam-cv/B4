using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class Utils : MonoBehaviour
{
    public Dictionary<string, CropType> crops = new Dictionary<string, CropType>();
    public static Utils Instance { get; private set; }
    public GameObject contenedorMaices;
    public GameObject[] maices = new GameObject[4];
    public GameObject[] tomates = new GameObject[4];
    public GameObject[] cebadas = new GameObject[4];
    public GameObject[] cañas = new GameObject[4];

    void Start()
    {
        if (Instance == null)
        {
            Instance = this;
        }
         //un for que inicialice los arrays
        for (int i = 0; i < 4; i++)
        {
            maices[i] = GameObject.Find("Maices" + i);
            tomates[i] = GameObject.Find("tomates" + i);
            cebadas[i] = GameObject.Find("cebadas" + i);
            cañas[i] = GameObject.Find("Cañas" + i);
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
        int cont = 0;
        foreach (Plot plot in plots)
        {
            if (plot.crop_type_id != null)
            {
                if (GameObject.Find("Click" + cont).GetComponent<SelectParcela>().planted == false)
                {
                    prueba.instance.indiceEtapa = GetGrowth(plot);

                    //switch crop_type_id
                    switch (plot.crop_type_id)
                    {
                        case "tomate":
                            contenedorMaices = GameObject.Find("tomates0");  //tomates[cont];
                            break;
                        case "cana":
                            contenedorMaices = cañas[cont];
                            break;
                        case "maiz":
                            contenedorMaices = GameObject.Find("Maices1"); //maices[cont];
                            break;
                        case "cebada":
                            contenedorMaices = cebadas[cont];
                            break;
                    }
                    //Guarda el contenedorMaices en el Queue de CultivosPlantados
                    GameObject.Find("Click" + cont).GetComponent<SelectParcela>().planted = true;
                    CultivosPlantados.instance.queueCultivos.Enqueue(contenedorMaices);

                    // Recorre todos los hijos del contenedor
                    foreach (Transform hijo in contenedorMaices.transform)
                    {
                        // Obtener el componente CrecimientoMaiz del hijo actual
                        prueba crecimientoMaiz = hijo.GetComponent<prueba>();

                        // Verificar si se encontró el componente CrecimientoMaiz
                        if (crecimientoMaiz != null)
                        {
                            // Iniciar el crecimiento del maíz
                            crecimientoMaiz.IniciarCrecimiento();
                        }
                        else
                        {
                            Debug.LogWarning("No se encontró el componente CrecimientoMaiz en un hijo del contenedor.");
                        }
                    }

                    //quantity = plot.quantity;
                }
            cont++;
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
