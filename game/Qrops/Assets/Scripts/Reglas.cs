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

    public void ToggleCanvasAndPlay()
    {
        ToggleCanvas(); // Llama a la función ToggleCanvas para cambiar de canvas
        // Establece Time.timeScale a 1 para reanudar el juego
        Time.timeScale = 1;
    }
}
