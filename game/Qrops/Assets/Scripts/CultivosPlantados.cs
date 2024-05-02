using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CultivosPlantados : MonoBehaviour
{
    public Queue<GameObject> queueCultivos = new Queue<GameObject>();
    //array de 4 gameobjects
    public GameObject[] cultivos = new GameObject[4];
    public Queue<int> queuePlots = new Queue<int>();
    public bool tienda = false;
    //crea la instancia de la clase
    public static CultivosPlantados instance;

    // Start is called before the first frame update
    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null) {
            instance = this;
        }
    }
    void update()
    {
        if (tienda)
        {
            GameObject.Find("Click0").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click1").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click2").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click3").GetComponent<Collider2D>().enabled = false;
        }
        else
        {
            GameObject.Find("Click0").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click1").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click2").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click3").GetComponent<Collider2D>().enabled = true;
        }
    }
}
