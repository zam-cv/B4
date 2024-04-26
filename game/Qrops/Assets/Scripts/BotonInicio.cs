using UnityEngine;

public class BotonInicio : MonoBehaviour
{
    public GameObject contenedorMaices; // Referencia al contenedor que contiene todos los maíces
    //Array de gameonjects
    public GameObject[] maices = new GameObject[4];
    public GameObject[] tomates = new GameObject[4];
    public GameObject[] cebadas = new GameObject[4];
    public GameObject[] cañas = new GameObject[4];

    private void Start()
    {
        //un for que inicialice los arrays
        for (int i = 0; i < 4; i++)
        {
            maices[i] = GameObject.Find("Maices" + i);
            tomates[i] = GameObject.Find("tomates" + i);
            cebadas[i] = GameObject.Find("cebadas" + i);
            cañas[i] = GameObject.Find("Cañas" + i);
        }
    }

    // Método que se llama cuando se hace clic en el botón
    public void OnClick()
    {
        //Checar el nombre del boton clickeado y hacer algo si es maiz_btn, tomato_btn, cebada_btn o caña_btn

        switch (gameObject.name)
        {
            case "maiz_btn":
                contenedorMaices = maices[ParcelaActual.instance.NumeroParcela - 1];
                break;
            case "tomato_btn":
                contenedorMaices = tomates[ParcelaActual.instance.NumeroParcela - 1];
                break;
            case "cebada_btn":
                contenedorMaices = cebadas[ParcelaActual.instance.NumeroParcela - 1];
                break;
            case "caña_btn":
                contenedorMaices = cañas[ParcelaActual.instance.NumeroParcela - 1];
                break;
        }
        //Guarda el contenedorMaices en el Queue de CultivosPlantados
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
    }
}
