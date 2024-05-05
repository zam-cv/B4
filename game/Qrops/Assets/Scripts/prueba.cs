using UnityEngine;

/* 
 * Esta clase se encarga de manejar el crecimiento de la planta.
 */

public class prueba : MonoBehaviour
{
    public Sprite[] etapasCrecimiento;
    private SpriteRenderer spriteRenderer;
    //private int etapaActual = 0;
    private bool crecimientoIniciado = false;
    //private float tiempoInicio;
    public GameObject panelMensaje;
    public int indiceEtapa;
    //crea la instancia de la clase
    public static prueba instance;


    // Esta función hace que se inicie el juego con la primera etapa de crecimiento
    void Start()
    {
        //Si la instancia es nula, se le asigna el valor de esta clase
        if (instance == null)
        {
            instance = this;
        }
        spriteRenderer = GetComponent<SpriteRenderer>();
        spriteRenderer.sprite = null;
        panelMensaje.SetActive(false);
    }
    
    // Hace que la planta crezca a través de las etapas de crecimiento
    void Update()
    {
        if (crecimientoIniciado)
        {
            //float tiempoTranscurrido = Time.time - tiempoInicio;
            // indiceEtapa = Mathf.FloorToInt(tiempoTranscurrido / 5f * etapasCrecimiento.Length);
            // indiceEtapa = Mathf.Clamp(indiceEtapa, 0, etapasCrecimiento.Length - 1);
            indiceEtapa = 3;
            spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
            if (indiceEtapa == etapasCrecimiento.Length - 1 && !panelMensaje.activeSelf)
            {
                crecimientoIniciado = false;
                panelMensaje.SetActive(true);
            }
        }
    }

    // Inicia el crecimiento de la planta
    public void IniciarCrecimiento()
    {
        if (!crecimientoIniciado)
        {
            //tiempoInicio = Time.time;
            //etapaActual = 0;
            spriteRenderer.sprite = etapasCrecimiento[indiceEtapa];
            print("indiceEtapa: " + indiceEtapa);
            crecimientoIniciado = true;
            panelMensaje.SetActive(false);
        }
    }

    // Métodos públicos para manipular el estado desde otros scripts
    public void ResetCrecimiento()
    {
        spriteRenderer.sprite = null;
        crecimientoIniciado = false;
    }

    public bool EstaCreciendo()
    {
        return crecimientoIniciado;
    }
}
