using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CultivosPlantados : MonoBehaviour
{
    public Queue<GameObject> queueCultivos = new Queue<GameObject>();
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
        
    }
}
