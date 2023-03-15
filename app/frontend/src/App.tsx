import Footer from './components/organizations/footer/Footer';
import Header from './components/organizations/header/Header';
import Main from './components/organizations/main/Main';
import Sidebar from './components/organizations/sidebar/Sidebar';
import styles from './App.module.scss';

function App() {
  return (
    <div className={styles.app}>
      <div className={styles.header}>
        <Header />
      </div>
      <div className={styles.sidebar}>
        <Sidebar />
      </div>
      <div className={styles.main}>
        <Main />
      </div>
      <div className={styles.footer}>
        <Footer />
      </div>
    </div>
  );
}

export default App;
