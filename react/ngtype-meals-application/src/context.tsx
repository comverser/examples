import React, { useContext, useEffect, useState } from "react";

import axios, { AxiosError } from "axios";

type MealType = {
  idMeal: string;
  strMeal: string;
  strMealThumb: string;
  strInstructions: string;
  strSource: string;
};

type AppContextType = {
  loading: boolean;
  meals: Array<MealType>;
  setSearchTerm: React.Dispatch<React.SetStateAction<string>>;
  fetchRandomMeal: () => void;
  showModal: boolean;
  selectedMeal: MealType;
  selectMeal: (idMeal: string, favoriteMeal: boolean) => void;
  closeModal: () => void;
  addToFavorites: (idMeal: string) => void;
  removeFromFavorites: (idMeal: string) => void;
  favorites: Array<MealType>;
};

const AppContext = React.createContext<AppContextType | null>(null);

const allMealsUrl = "https://www.themealdb.com/api/json/v1/1/search.php?s=a";
const randomMealUrl = "https://www.themealdb.com/api/json/v1/1/random.php";

const getFavoritesFromLocalStorage = () => {
  const localStorageFavorites = localStorage.getItem("favorites");

  if (localStorageFavorites) {
    return JSON.parse(localStorage.getItem("favorites")!);
  } else {
    return [];
  }
};

const AppProvider = ({ children }: { children: JSX.Element }) => {
  const [loading, setLoading] = useState(false);
  const [meals, setMeals] = useState<Array<MealType>>([]);
  const [searchTerm, setSearchTerm] = useState("");

  const [showModal, setShowModal] = useState(false);
  const [selectedMeal, setSelectedMeal] = useState<MealType>({
    idMeal: "",
    strMeal: "",
    strMealThumb: "",
    strInstructions: "",
    strSource: "",
  });
  const [favorites, setFavorites] = useState<Array<MealType>>(
    getFavoritesFromLocalStorage()
  );

  const fetchMeals = async (url: string) => {
    setLoading(true);
    try {
      const { data } = await axios(url);
      if (data.meals) setMeals(data.meals);
      else setMeals([]);
    } catch (error: unknown) {
      if (error instanceof AxiosError) console.error(error.response);
      else console.error(error);
    }
    setLoading(false);
  };

  const fetchRandomMeal = () => {
    fetchMeals(randomMealUrl);
  };

  const selectMeal = (idMeal: string, favoriteMeal: boolean) => {
    let meal: MealType;
    if (favoriteMeal) {
      meal = favorites.find((meal) => meal.idMeal === idMeal)!;
    } else {
      meal = meals.find((meal) => meal.idMeal === idMeal)!;
    }
    setSelectedMeal(meal);
    setShowModal(true);
  };

  const closeModal = () => {
    setShowModal(false);
  };

  const addToFavorites = (idMeal: string) => {
    const alreadyFavorite = favorites.find((meal) => meal.idMeal === idMeal);
    if (alreadyFavorite) return;
    const meal = meals.find((meal) => meal.idMeal === idMeal);
    const updatedFavorites = [...favorites, meal!];
    setFavorites(updatedFavorites);
    localStorage.setItem("favorites", JSON.stringify(updatedFavorites));
  };
  const removeFromFavorites = (idMeal: string) => {
    const updatedFavorites = favorites.filter((meal) => meal.idMeal !== idMeal);
    setFavorites(updatedFavorites);
    localStorage.setItem("favorites", JSON.stringify(updatedFavorites));
  };

  useEffect(() => {
    fetchMeals(allMealsUrl);
  }, []);

  useEffect(() => {
    if (!searchTerm) return;
    fetchMeals(`${allMealsUrl}${searchTerm}`);
  }, [searchTerm]);

  return (
    <AppContext.Provider
      value={{
        loading,
        meals,
        setSearchTerm,
        fetchRandomMeal,
        showModal,
        selectedMeal,
        selectMeal,
        closeModal,
        addToFavorites,
        removeFromFavorites,
        favorites,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useGlobalContext = () => {
  return useContext(AppContext);
};

export { AppContext, AppProvider };
