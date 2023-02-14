// Basic Title 
import {TitleProps} from '../interfaces/AtomProps';

export default function TitleV2(props: TitleProps) {
    return (
        <h1 className="font-thin text-2xl font-fira-mono">{props.title}</h1>
    );
}
