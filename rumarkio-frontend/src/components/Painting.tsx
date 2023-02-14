import { PaintingProps } from '../interfaces/CompProps';
import styles from './Painting.module.css';

export default function Painting(props: PaintingProps) {
    // check if there is valid image 
    // if there is return painting with img tag
  return (
    <div className={styles.painting}>
      <img src={props.img} alt={props.description} />
      <p className="font-fira-mono text-center text-sm mt-2">
        {props.description}
      </p>
    </div>
 
);
}
