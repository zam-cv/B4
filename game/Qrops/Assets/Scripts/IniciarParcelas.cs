using System.Collections;
using System.Collections.Generic;
using UnityEngine;

/* 
 * Esta clase se encarga de iniciar las parcelas con los cultivos
 * que se encuentran en ellas.
 */

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
    //Crea un 4 plots y dale valores
    public Plot plot1 = new Plot { crop_type_id = "1", quantity = 1, growth = 2 };
    public Plot plot2 = new Plot { crop_type_id = "2", quantity = 1, growth = 2 };
    public Plot plot3 = new Plot { crop_type_id = "3", quantity = 1, growth = 0 };
    public Plot plot4 = new Plot { crop_type_id = "4", quantity = 1, growth = 1 };
    
    // Esta función añade los plots a la lista
    void Start(){
        //Agrega los plots a la lista
        plots.Add(plot1);
        plots.Add(plot2);
        plots.Add(plot3);
        plots.Add(plot4);
        //
    }
}
