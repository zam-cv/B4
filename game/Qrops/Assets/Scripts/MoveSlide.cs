using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.UIElements;

/* 
 * Esta clase se encarga de mover los slides de la pantalla de inicio
 * hacia la izquierda y derecha cuando se hace clic en los botones
 * de siguiente y anterior.
 */

public class MoveSlide : MonoBehaviour
{
    // Declare array of panels to store the slides
    public UnityEngine.UI.Image[] slides;

    public UnityEngine.UI.Button nextButton;
    public UnityEngine.UI.Button previousButton;
    private float[] centerPositions;
    private float[] leftPositions;
    private float[] rightPositions;
    
    private int currentSlide = 0;

    private float slideWidth = 2000f;
    private float transition = 0.3f;



    // Start is called before the first frame update
    void Start()
    {
        // Assign the position of each slide to the array
        slides = new UnityEngine.UI.Image[5];
        centerPositions = new float[5];
        leftPositions = new float[5];
        rightPositions = new float[5];

        nextButton = GameObject.Find("Next").gameObject.GetComponent<UnityEngine.UI.Button>();
        previousButton = GameObject.Find("Prev").gameObject.GetComponent<UnityEngine.UI.Button>();

        for(int i = 0; i < 5; i++)
        {
            slides[i] = GameObject.Find("SS_" + (i + 1).ToString()).gameObject.GetComponent<UnityEngine.UI.Image>();

            centerPositions[i] = slides[i].transform.position.x;
            leftPositions[i] = centerPositions[i] - slideWidth;
            rightPositions[i] = centerPositions[i] + slideWidth;
        }
        
        print(centerPositions[0]);
        // Set slides to be on the right side of the screen
        for(int i = 0; i < 5; i++)
        {
            slides[i].transform.position = new Vector2(rightPositions[i], slides[i].transform.position.y);
        }      

        // Move the first slide to the center of the screen
        StartCoroutine(MoveForward(0));
        UpdateButtons();
    }

    // Cambia la posición de los slides
    public void UpdateButtons(){
        //Hide the next button if the last slide is reached
        nextButton.gameObject.SetActive(currentSlide < 4);

        //Hide the previous button if the first slide is reached
        previousButton.gameObject.SetActive(currentSlide > 0);
    }

    // Mueve el slide actual a la izquierda
    public void NextSlide(){

        if(currentSlide < 4)
        {
            StartCoroutine(MoveLeft(currentSlide));
            currentSlide++;
            StartCoroutine(MoveForward(currentSlide));
        }
        UpdateButtons();
    }

    // Mueve el slide actual a la derecha
    public void PreviousSlide(){

        if(currentSlide > 0)
        {
            StartCoroutine(MoveRight(currentSlide));
            currentSlide--;
            StartCoroutine(MoveBackward(currentSlide));
        }
        UpdateButtons();
    }
 
    // Corrutina para mover el slide hacia adelante
    IEnumerator MoveForward(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime/transition;
            slides[slide].transform.position = Vector2.Lerp(new Vector2(rightPositions[slide], slides[slide].transform.position.y), new Vector2(centerPositions[slide], slides[slide].transform.position.y), t);
            yield return null;
        }
    }

    // Corrutina para mover el slide hacia atrás
    IEnumerator MoveBackward(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime/transition;
            slides[slide].transform.position = Vector2.Lerp(new Vector2(leftPositions[slide], slides[slide].transform.position.y), new Vector2(centerPositions[slide], slides[slide].transform.position.y), t);
            yield return null;
        }
    }

    // Corrutina para mover el slide a la izquierda
    IEnumerator MoveLeft(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime/transition;
            slides[slide].transform.position = Vector2.Lerp(new Vector2(centerPositions[slide], slides[slide].transform.position.y), new Vector2(leftPositions[slide], slides[slide].transform.position.y), t);
            yield return null;
        }
    }

    // Corrutina para mover el slide a la derecha
    IEnumerator MoveRight(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime/transition;
            slides[slide].transform.position = Vector2.Lerp(new Vector2(centerPositions[slide], slides[slide].transform.position.y), new Vector2(rightPositions[slide], slides[slide].transform.position.y), t);
            yield return null;
        }
    }



    



    // Update is called once per frame
    void Update()
    {
        
    }
}
