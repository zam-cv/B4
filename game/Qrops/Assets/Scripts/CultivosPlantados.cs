using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CultivosPlantados : MonoBehaviour
{
    //array de 4 gameobjects
    public GameObject[] parcelas = new GameObject[4];
    //array de 4 scripts
    public Queue<GameObject> queueCultivos = new Queue<GameObject>();
    //crea la instancia de la clase
    public static CultivosPlantados instance;

    // Start is called before the first frame update
    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null) {
            instance = this;
        }
        parcelas[0] = GameObject.Find("Click0");
        parcelas[1] = GameObject.Find("Click1");
        parcelas[2] = GameObject.Find("Click2");
        parcelas[3] = GameObject.Find("Click3");
    }
    void update()
    {
        parcelas[0].GetComponent<SelectParcela>().planted = true;
        parcelas[1].GetComponent<SelectParcela>().planted = true;
        parcelas[2].GetComponent<SelectParcela>().planted = true;
        parcelas[3].GetComponent<SelectParcela>().planted = true;
    }
}
