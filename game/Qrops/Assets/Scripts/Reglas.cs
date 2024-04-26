using UnityEngine;

public class Reglas : MonoBehaviour
{
    public GameObject firstCanvas;
    public GameObject secondCanvas;

    public void ToggleCanvas()
    {
        // Verifica si el primer Canvas está activo
        if (firstCanvas.activeSelf)
        {
            firstCanvas.SetActive(false);
            secondCanvas.SetActive(true);
            //print("Cambiando a segundo canvas (tienda)");
            GameObject.Find("Click0").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click1").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click2").GetComponent<Collider2D>().enabled = false;
            GameObject.Find("Click3").GetComponent<Collider2D>().enabled = false;
        }
        else
        {
            firstCanvas.SetActive(true);
            secondCanvas.SetActive(false);
            //print("Cambiando a primer canvas (juego)");
            GameObject.Find("Click0").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click1").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click2").GetComponent<Collider2D>().enabled = true;
            GameObject.Find("Click3").GetComponent<Collider2D>().enabled = true;
        }
    }
}
