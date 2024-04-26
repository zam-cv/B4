using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class IniciarParcelas : MonoBehaviour
{
    public struct Plot
    {
        public string crop_type_id;
        public int quantity;
        public int growth;
    }
    
    //Crea una lista de plots publica
    public List<Plot> plots = new List<Plot>();
    //Crea un plot y dale valores
    public Plot plot1 = new Plot { crop_type_id = "1", quantity = 1, growth = 2 };

    void Start(){

    }
}
