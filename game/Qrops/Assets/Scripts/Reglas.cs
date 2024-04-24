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
        }
        else
        {
            firstCanvas.SetActive(true);
            secondCanvas.SetActive(false);
        }
    }
}
