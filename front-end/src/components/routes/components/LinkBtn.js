import React from 'react';
import '../../../styles/App.css';

import axios from 'axios';
import { Link} from 'react-router-dom';

class LinkBtn extends React.Component {

    //Default values displayed when the server is unable to send data over
    constructor(props) {
        super(props);
        this.state = {
            heading:"Orbital Bodies",
            dice: "1d5",
            results: [{roll:1, value:"gas giant"}, {roll:2, value:"nebula"}, {roll:3, value:"star"}, {roll:4, value:"black hole"}, {roll:"5-6", value: "rangeTest"}]
        }

        this.getServerData = this.getServerData.bind(this);
    }

    //Receives data from server and sets table values to the received data
    getServerData(url) {
        axios.get(url).then((response) => {
            this.setState ({
                heading: response.data.heading,
                dice: response.data.dice.amount + "d" + response.data.dice.size,
                results: response.data.results,
            })
        })
    }

    //Sets the URL where the data is received from
    componentDidMount() {
        this.getServerData(this.props.url);
    }

    render() {
        return (
            <Link to = {{ pathname: "/roll",
                          state: { title: this.props.btnName,
                                   heading: this.state.heading,
                                   dice: this.state.dice,
                                   results: this.state.results,
                        }}}>
                <button>
                    {this.props.btnName}
                </button>
            </Link>
        )
    }
}
export default LinkBtn;
