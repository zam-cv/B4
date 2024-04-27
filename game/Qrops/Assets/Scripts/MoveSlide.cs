using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class MoveSlide : MonoBehaviour
{
    // Declare array of panels to store the slides
    public GameObject[] slides;
    // Declare array of RectTransform to store the initial position of each slide
    private RectTransform[] slideInsidePositions;
    private RectTransform[] slideLeftPositions;
    private RectTransform[] slideRightPositions;
    
    private int currentSlide = 0;



    // Start is called before the first frame update
    void Start()
    {
        // Assign the position of each slide to the array
        slides = new GameObject[5];
        slideInsidePositions = new RectTransform[5];
        slideLeftPositions = new RectTransform[5];
        slideRightPositions = new RectTransform[5];

        for(int i = 0; i < 5; i++)
        {
            slides[i] = GameObject.Find("SS_" + (i + 1).ToString());
            print(slides[i].name);

            // get rect transform of the slide
            slideInsidePositions[i] = slides[i].GetComponent<RectTransform>();
            print(slideInsidePositions[i].position.x);
            slideLeftPositions[i] = slides[i].GetComponent<RectTransform>();
            slideLeftPositions[i].position = new Vector3(slideInsidePositions[i].position.x - 2000, slideInsidePositions[i].position.y, slideInsidePositions[i].position.z);
            slideRightPositions[i] = slides[i].GetComponent<RectTransform>();
            slideRightPositions[i].position = new Vector3(slideInsidePositions[i].position.x + 2000, slideInsidePositions[i].position.y, slideInsidePositions[i].position.z);
        }
        
        // Set slides to be on the right side of the screen
        for(int i = 0; i < 5; i++)
        {
            print("right");
            print(slideRightPositions[i].position);
            slides[i].transform.position = slideRightPositions[i].position;
        }      

        // Move the first slide to the center of the screen
        currentSlide = 0;
        NextSlide();
    }

    public void NextSlide(){
        if(currentSlide < 4)
        {
            StartCoroutine(MoveLeft(currentSlide));
            currentSlide++;
        }
    }


    IEnumerator MoveLeft(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime;
            slides[slide].transform.position = Vector3.Lerp(slideInsidePositions[slide].position, slideLeftPositions[slide].position, t);
            yield return null;
        }
    }

    IEnumerator MoveRight(int slide)
    {
        float t = 0.0f;
        while (t < 1.0f)
        {
            t += Time.deltaTime;
            slides[slide].transform.position = Vector3.Lerp(slideInsidePositions[slide].position, slideRightPositions[slide].position, t);
            yield return null;
        }
    }



    



    // Update is called once per frame
    void Update()
    {
        
    }
}
