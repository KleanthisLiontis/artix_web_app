import React, { Component } from 'react';
import axios from "axios";

class CreateToDoItem extends Component {
    state = {
        title: ""
    }
    createItem = () => {
        axios.post("http://127.0.0.1:8000/v1/item/create" + this.state.title,
        {},
        {headers: {"token": "some_token"}})
        .then(response => response {
            this.setState({"title": ""});
            this.props.passBackResponse(response);
        });
    }

    handleTitleChange = (e) => {
        this.setState({"title": e.target.value});
    }
    render() {
        return (
            . . .
        )
    }
}
export default CreateToDoItem;