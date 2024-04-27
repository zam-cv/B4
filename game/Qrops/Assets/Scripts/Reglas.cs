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
            //toggle CultivosPlantados.tienda
            CultivosPlantados.instance.tienda = true;
            //print("Cambiando a segundo canvas (tienda)");
        }
        else
        {
            firstCanvas.SetActive(true);
            secondCanvas.SetActive(false);
            //print("Cambiando a primer canvas (juego)");
            CultivosPlantados.instance.tienda = false;
        }
    }
}
