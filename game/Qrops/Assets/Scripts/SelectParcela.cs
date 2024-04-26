using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SelectParcela : MonoBehaviour
{
    public bool planted = false;
    //crea la instancia de la clase
    public static SelectParcela instance;
    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null) {
            instance = this;
        }
        GameObject.Find("select1").GetComponent<SpriteRenderer>().enabled = false;
        GameObject.Find("select2").GetComponent<SpriteRenderer>().enabled = false;
        GameObject.Find("select3").GetComponent<SpriteRenderer>().enabled = false;
        GameObject.Find("select4").GetComponent<SpriteRenderer>().enabled = false;
    }

    void OnMouseDown(){
    Debug.Log("GameObject ha sido clickeado");
    //Si el gameobject se llama "Click" se accede a la variable NumeroParcela de ParcelaActual y se le da el valor de 1
    switch(gameObject.name){
        case "Click0":
            ParcelaActual.instance.NumeroParcela = 1;
            GameObject.Find("select1").GetComponent<SpriteRenderer>().enabled = true;
            GameObject.Find("select2").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select3").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select4").GetComponent<SpriteRenderer>().enabled = false;
            break;
        case "Click1":
            ParcelaActual.instance.NumeroParcela = 2;
            GameObject.Find("select1").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select2").GetComponent<SpriteRenderer>().enabled = true;
            GameObject.Find("select3").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select4").GetComponent<SpriteRenderer>().enabled = false;
            break;
        case "Click2":
            ParcelaActual.instance.NumeroParcela = 3;
            GameObject.Find("select1").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select2").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select3").GetComponent<SpriteRenderer>().enabled = true;
            GameObject.Find("select4").GetComponent<SpriteRenderer>().enabled = false;
            break;
        case "Click3":
            ParcelaActual.instance.NumeroParcela = 4;
            GameObject.Find("select1").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select2").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select3").GetComponent<SpriteRenderer>().enabled = false;
            GameObject.Find("select4").GetComponent<SpriteRenderer>().enabled = true;
            break;
    }
}

}